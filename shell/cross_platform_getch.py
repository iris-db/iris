class GetChar:
	def __init__(self):
		try:
			self.impl = _GetCharWindows()
		except ImportError:
			self.impl = _GetCharUnix()

	def __call__(self):
		return self.impl()


class _GetCharUnix:
	def __init__(self):
		import sys
		import tty
		import termios

	def __call__(self):
		import sys
		import tty
		import termios

		fd = sys.stdin.fileno()
		old_settings = termios.tcgetattr(fd)
		try:
			tty.setraw(sys.stdin.fileno())
			ch = sys.stdin.read(1)
		finally:
			termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)
		return ch


class _GetCharWindows:
	def __init__(self):
		import msvcrt

	def __call__(self):
		import msvcrt
		return msvcrt.getch()
