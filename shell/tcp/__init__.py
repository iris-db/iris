# Empty
import socket

_HOST = "127.0.0.1"


def connect_to_server():
	with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
		s.connect((_HOST, 16713))
		s.sendall(b"Hello world!")

		data = s.recv(1024)

	print("Received", repr(data))
