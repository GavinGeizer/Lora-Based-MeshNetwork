#include <ESP8266WiFi.h>

void setup()
{
    pinMode(BUILTIN_LED,OUTPUT)
    Serial.begin(115200)
    Serial.println(F("\nESP8266 Wifi Scanner example"))

    WiFi.mode(WIFI_STA)
}

void loop()
{
    digitalWrite(BUILTIN_LED,HIGH)
    delay(250)
    digitalWrite(BUILTIN_LED,LOW)
    delay(250)
}