package sys

// Builder dispatches a build action.
type Builder interface {
	Build() error // Build executes the build process.
}
