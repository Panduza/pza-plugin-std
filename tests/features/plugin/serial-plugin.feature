Feature: The driver serial_stream in Panduza platform must be able to transmit information between the client and the hardware device

  Background:
    Given the tested driver connect with the device
    Given a reactor connected on a test platform
    

  Scenario: Normal operation with tcp connection
    Given the bytes attribute rx "serial-stream/RX"
    Given the bytes attribute tx "serial-stream/TX"
    When I set tx bytes to "test 1"
    Then the rx bytes value is "echo : test 1"
    When I set tx bytes to "test 2"
    Then the rx bytes value is "echo : test 2"


