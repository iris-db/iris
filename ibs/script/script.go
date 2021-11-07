package script

import (
	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"
	"path"
	"strings"

	"github.com/estebangarcia21/subprocess"
	"github.com/iris-db/iris/scripts/cli"

	"gopkg.in/yaml.v2"
)

const (
	ConfigsDirectory     = "etc/ibs_scripts"
	ArtifactOutDirectory = "dist"
)

// Script is a runnable script that dispatches build or test operations.
type Script struct {
	Name   string // Name is the name of the script.
	Config Config // Config is the YML data.
}

// Config is the YML script specification.
type Config struct {
	Module           string   `yaml:"module,omitempty"`            // Module is where to run the process.
	RequiredCommands []string `yaml:"required-commands,omitempty"` // RequiredCommands are the required commands to run the process.
	Run              string   `yaml:"run,omitempty"`               // Run is the shell string to run.
	Artifacts        []string `yaml:"artifacts,omitempty"`         // Artifacts are the objects to collect after running the process.
}

// ExecutionStage is the current stage of the script running process.
type ExecutionStage string

var (
	// SetupStage verifies that the script exists and is in the proper YML format.
	SetupStage ExecutionStage = "Setup"
	// ValidationStage validates that all required commands are present in the system PATH.
	ValidationStage ExecutionStage = "Validation"
	// RunStage runs the script.
	RunStage ExecutionStage = "Run"
)

// New loads a Script from the filesystem based on the ConfigsDirectory
func New(opts *cli.Options) (Script, error) {
	modeTyped, err := ModeFromString(opts.Type)
	if err != nil {
		return Script{}, err
	}

	scriptName := opts.Name

	dir := fmt.Sprintf("%s/%s/%s.yml", ConfigsDirectory, modeTyped.ToString(), scriptName)

	file, err := ioutil.ReadFile(path.Join("..", dir))
	if err != nil {
		return Script{}, SetupError(fmt.Errorf("the script %s does not exist", scriptName))
	}

	cfg := Config{}
	if err := yaml.Unmarshal(file, &cfg); err != nil {
		return Script{}, SetupError(errors.New("improper script format. Please check the example at etc/ibs_script_example.yml"))
	}

	return Script{
		Name:   scriptName,
		Config: cfg,
	}, nil
}

// Start starts the script.
func (s Script) Start() error {
	origDir, err := os.Getwd()
	if err != nil {
		return SetupError(err)
	}

	if err := os.Chdir(".."); err != nil {
		return SetupError(err)
	}

	cfg := s.Config

	if err := os.Chdir(fmt.Sprintf("./%s", cfg.Module)); err != nil {
		return SetupError(fmt.Errorf("module directory %s does not exist! Please check that it is correct", cfg.Module))
	}

	modDir, err := os.Getwd()
	if err != nil {
		return SetupError(err)
	}

	// Verify all required commands exist.
	var cmds []string

	for _, cmd := range cfg.RequiredCommands {
		if _, err := exec.LookPath(cmd); err != nil {
			cmds = append(cmds, cmd)
		}
	}

	if len(cmds) != 0 {
		return ValidationError(fmt.Errorf("missing commands: %s", strings.Join(cmds, ", ")))
	}

	// Run the build steps.
	if cfg.Run != "" {
		sp := subprocess.New(s.Config.Run, subprocess.Shell)
		if err := sp.Exec(); err != nil {
			return err
		}
	}

	if err := os.Chdir(origDir); err != nil {
		return RunError(fmt.Errorf("could not into the original working directory (%s)", origDir))
	}

	for _, a := range cfg.Artifacts {
		binPath := path.Join(modDir, a)

		outDir := path.Join(ArtifactOutDirectory, s.Name)
		if err := os.MkdirAll(outDir, os.ModePerm); err != nil {
			return RunError(fmt.Errorf("could not create artifacts directory at %s", outDir))
		}

		atok := strings.Split(a, "/")
		dest := fmt.Sprintf("%s/%s", outDir, atok[len(atok)-1])

		if err := os.Rename(binPath, dest); err != nil {
			return RunError(fmt.Errorf("could not move artifact %s to %s", a, outDir))
		}
	}

	return nil
}
