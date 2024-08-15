from pymodbus.client import ModbusTcpClient
import threading
import time

# 接続先
HOST = '192.168.0.100'

# Modbus/TCPの定義
MODBUS_PORT = 5502


def read_input_registers():
    counter = 0
    try:
        client = ModbusTcpClient(host=HOST, port=MODBUS_PORT)
        while True:
            client.connect()
            time.sleep(0.05)
            response = client.read_input_registers(0x00, 0x02)

            print(response.registers)
            time.sleep(100.05)
            client.close()
            time.sleep(0.05)
            counter = counter+1
            if counter > 2:
                break
    except Exception as e:
        print(e)


print('start')
# 5つのクライアントスレッドを起動
threads = []
for i in range(51):

    t = threading.Thread(target=read_input_registers)
    threads.append(t)
    print('\tthread start')
    time.sleep(1)
    t.start()

# すべてのスレッドが終了するのを待つ
for t in threads:
    t.join()

time.sleep(120)
print("All threads finished.")

