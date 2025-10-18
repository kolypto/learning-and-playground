# IDF (std)

## RTOS Watchdog (IDF)

In `std` IDF, your tasks, including the `main()` function, must yield control
to FreeRTOS so that it can run other tasks.

If you don't, after 5s a watchdog will fire and kill the app with this message:

```
E (21573) task_wdt: Task watchdog got triggered. The following tasks/users did not reset the watchdog in time:
E (21573) task_wdt:  - IDLE (CPU 0)
E (21573) task_wdt: CPU 0: main
```

## WiFi (IDF)

When using IDF WiFi, it tries to register the chip (via DHCP client)
as "espressif", so the chip will often be available using hostname "espressif".

You can customize the hostname using `sdkconfig.defaults`:

  CONFIG_LWIP_LOCAL_HOSTNAME="esp32c3"

See all the options:
[Configuration Options Reference](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/api-reference/kconfig-reference.html)
