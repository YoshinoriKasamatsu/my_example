import socket
import struct
import function_code as function_code
import config


def create_modbus_request_header(
        transaction_id: int,
        protocol_id: int,
        length: int,
        unit_id: int):
    """Create Modbus Request Header
    Args:
        transaction_id (int): (トランザクション識別子, 2bytes)
        protocol_id (int): プロトコル識別子(2bytes)
        length (int): フィールド長(2bytes) = unit_id(1byte) + function_code(1byte) + data(可変長)
        unit_id (int): ユニット識別子(1byte) =スレーブ識別子
        Returns:
            bytes: Modbus Request Header
    """

    # Transaction ID (2 bytes), Protocol ID (2 bytes), Length (2 bytes), Unit ID (1 byte)
    header = struct.pack('>HHHb', transaction_id, protocol_id, length, unit_id)
    return header


def create_modbus_request_body(
        function_code: int,
        starting_address: int,
        quantity_of_registers: int):
    """Create Modbus Request Body
    Args:
        function_code (int): Function Code
        starting_address (int): Starting Address
        quantity_of_registers (int): Quantity of Registers
        Returns:
            bytes: Modbus Request Body
    """
    # Function Code (1 byte), Starting Address (2 bytes), Quantity of Registers (2 bytes)
    body = struct.pack('>bHH', function_code, starting_address, quantity_of_registers)
    return body


def build_modbus_request():
    # Transaction ID (2 bytes), Protocol ID (2 bytes), Length (2 bytes), Unit ID (1 byte)
    header = create_modbus_request_header(
        transaction_id=config.TRANSACTION_ID,
        protocol_id=config.PROTOCOL_ID,
        length=0x0006,
        unit_id=config.CLIENT_ID)

    # Function Code (1 byte), Starting Address (2 bytes), Quantity of Registers (2 bytes)
    print(function_code.FunctionCode.READ_INPUT_REGISTERS)

    payload = create_modbus_request_body(
        function_code=0x04,
        starting_address=0x0000,
        quantity_of_registers=0x0001)

    return header + payload


def send_modbus_request(host, port):
    request = build_modbus_request()
    print(f"Request: {request}")

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        print(f"Connecting to {host}:{port}")
        s.connect((host, port))
        s.sendall(request)

        response = s.recv(1024)
        print(f"Response: {response}")

        # 応答からデータ部分を取り出す
        data = response[9:]
        print(f"Response: {data}")
        if len(data) == 2 + 1:
            value = struct.unpack('>H', data[1:])[0]
            return value


if __name__ == "__main__":
    register_value = send_modbus_request(config.HOST, config.MODBUS_PORT)
    if register_value is not None:
        print(f"Value of Holding Register 40001: {register_value}")
