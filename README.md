# winctl

Command line tool to quickly change some Windows settings.

### Features

* Change display topology in multi-monitor setup.

  ```sh
  winctl.exe display internal # only enable main monitor.
  winctl.exe display external # only enable second monitor.
  winctl.exe display clone
  winctl.exe display extend
  ```

* Change default audio output device.

  ```sh
  winctl.exe audio 0 # change to 0-th audio device.
  ```
