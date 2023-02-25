# Rusteam

A partial implementation of the IAPWS-IF97 steam table calculations in Rust.

There's still a lot of work to be done, but we'll get there eventually :)

Reference: http://www.iapws.org/relguide/IF97-Rev.html

## Progress

Here you can find the current status of each of the calculations implemented so far. Some of them have been implemented, but not thoroughly tested, so use them at your own risk.

### Region 1

- **Forward equations**

| Property | Implemented | Tested |
|:---------|:--------------:|:-------:|
|Specific volume                 |**OK**| No |
|Specific enthalpy               |**OK**| **OK**|
|Specific internal energy        |**OK**| No |
|Specific entropy                |**OK**| No |
|Specific isobaric heat capacity |**OK**| No |


### Region 4

| Property | Implemented | Tested |
|:---------|:--------------:|:-------:|
|Saturation pressure |**OK**| **OK** |
|Saturation temperature |**OK**| **OK** |


