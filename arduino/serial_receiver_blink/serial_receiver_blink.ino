#define BUFFER_SIZE 5

byte buffer[BUFFER_SIZE];

void setup() {
  // initialize digital pin LED_BUILTIN as an output.
  pinMode(LED_BUILTIN, OUTPUT);
  // Initialize Serial port
  Serial.begin(57600);
}

/*
* Blink 1Hz <times> times
*/
void blink(int times) {
  for (int i = 0; i < times; i++) {
    digitalWrite(LED_BUILTIN, HIGH);  // turn the LED on (HIGH is the voltage level)
    delay(1000);                      // wait for a second
    digitalWrite(LED_BUILTIN, LOW);   // turn the LED off by making the voltage LOW
    delay(1000);                      // wait for a second
  }
}

void loop() {
  // put your main code here, to run repeatedly:
  int bytes_read = 0;
  bytes_read = Serial.readBytes(buffer, BUFFER_SIZE);
  if (bytes_read != 0) {
    if (buffer[0] > 0x40) {
      blink(buffer[0] - 0x40);
    }


    // char output[80];
    // for (int b = 0; b < bytes_read; b++) {
    //   sprintf(output, "Received(%d): %02x", b, buffer[b]);
    //   Serial.println(output);
    // }
  }
}
