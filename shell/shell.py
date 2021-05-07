import platform
import sys

from cross_platform_getch import GetChar

system = platform.system()

getch = GetChar()

if system == "Linux" or system == "Darwin":
	def get_key():
		first_char = getch()
		if first_char == '\x1b':
			return {"[A": "up", "[B": "down", "[C": "right", "[D": "left"}[getch() + getch()]
		else:
			return first_char
elif system == "Windows":
	def get_key():
		first_char = getch()
		if first_char == b'\xe0':
			return {"H": "up", "P": "down", "M": "right", "K": "left"}[getch()]
		else:
			return first_char


def main():
	for i in range(0, 20):
		key = get_key()

		if ord(key) == 127:
			print("\b \b", end="")
		else:
			print(key, end="")

		sys.stdout.flush()


if __name__ == "__main__":
	main()
