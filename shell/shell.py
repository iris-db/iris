import platform
import sys
import colorama

from cross_platform_getch import GetChar

system = platform.system()

getch = GetChar()

if system == "Linux" or system == "Darwin":
	def get_key():
		c1 = getch()

		if c1 == "\x1b":
			c2 = getch()
			if c2 == 127:
				return "CMD_BACKSPACE"
			else:
				return {"[A": "up", "[B": "down", "[C": "right", "[D": "left"}[c2 + getch()]
		else:
			return c1
elif system == "Windows":
	def get_key():
		c1 = getch()
		if c1 in ("\x00", "\xe0"):
			arrows = {"H": "up", "P": "down", "M": "right", "K": "left"}
			c2 = getch()
			return arrows.get(c2, c1 + c2)
		else:
			return c1
else:
	print(f"Could not start shell. Unsupported platform {system}")
	exit(1)


def ctrl_key(key):
	"""Checks if a key is pressed with along with control"""
	return ord(key) & 0x1f


def main():
	colorama.init()

	prompt = ""

	min_index = len(prompt)

	current_line = prompt
	index = min_index

	sys.stdout.write(prompt)
	sys.stdout.flush()

	# sys.stdout.write(u"\u001b[" + str(min_index) + "C")
	# sys.stdout.flush()

	while True:
		key = get_key()

		if len(key) == 1:
			code = ord(key)

			if code == ctrl_key('c'):
				break
			elif 32 <= code <= 126:
				current_line = current_line[:index] + chr(code) + current_line[index:]
				index += 1
			elif code == 127:
				current_line = current_line[:index - 1] + current_line[index:]
				index = max(0, index - 1)
			elif code in {10, 13}:
				sys.stdout.write("\n")

				print(current_line)

				current_line = ""
				index = min_index
		else:
			if key == "left":
				index = max(min_index, index - 1)
			elif key == "right":
				index = min(len(current_line), index + 1)
			else:
				pass

		sys.stdout.write("\33[2K\r")
		sys.stdout.write(u"\u001b[0K")
		sys.stdout.write(prompt + current_line)

		if index > min_index:
			sys.stdout.write("\r")
			sys.stdout.write(u"\u001b[" + str(index) + "C")

		sys.stdout.flush()


if __name__ == "__main__":
	main()
