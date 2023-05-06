# Rusteam

A partial implementation of the IAPWS-IF97 steam table calculations in Rust.

There's still a lot of work to be done, but we'll get there eventually :)

Reference: http://www.iapws.org/relguide/IF97-Rev.html

I've also drawn some inspiration from the C++ library CoolProp: https://github.com/CoolProp/IF97

## Progress

Here you can find the current status of each of the calculations implemented so far. Some of them have been implemented, but not thoroughly tested, so use them at your own risk.

### Region 1

- **Forward equations**

| Property | Implemented | Tested |
|:---------|:--------------:|:-------:|
|Specific volume                 |:heavy_check_mark:| :heavy_check_mark: |
|Specific internal energy        |:heavy_check_mark:| :heavy_check_mark: |
|Specific entropy                |:heavy_check_mark:| :heavy_check_mark: |
|Specific enthalpy               |:heavy_check_mark:| :heavy_check_mark: |
|Specific isobaric heat capacity |:heavy_check_mark:| :heavy_check_mark: |
|Specific isochoric heat capacity |:x:| :x: |
|Speed of sound|:x:| :x: |

- **Backward equations**

| Equation | Implemented | Tested |
|:---------|:--------------:|:-------:|
|T(p,h)                 |:heavy_check_mark:| :heavy_check_mark: |
|T(p,s)                 |:heavy_check_mark:| :heavy_check_mark: |


### Region 2

- **Forward equations**

| Property | Implemented | Tested |
|:---------|:--------------:|:-------:|
|Specific volume                 |:heavy_check_mark:| :heavy_check_mark: |
|Specific internal energy        |:heavy_check_mark:| :heavy_check_mark: |
|Specific entropy                |:heavy_check_mark:| :heavy_check_mark: |
|Specific enthalpy               |:heavy_check_mark:| :heavy_check_mark: |
|Specific isobaric heat capacity |:heavy_check_mark:| :heavy_check_mark: |
|Specific isochoric heat capacity |:x:| :x: |
|Speed of sound |:x:| :x: |

- **Backward equations**

| Equation | Implemented | Tested |
|:---------|:--------------:|:-------:|
|T(p,h)                |:x:| :x: |
|T(p,s)                 |:x:| :x: |

### Region 3

- **Forward equations**

| Property | Implemented | Tested |
|:---------|:--------------:|:-------:|
|Pressure|:x:| :x: |
|Specific internal energy|:x:| :x: |
|Specific entropy|:x:| :x: |
|Specific enthalpy|:x:| :x: |
|Specific isochoric heat capacity |:x:| :x: |
|Specific isobaric heat capacity |:x:| :x: |
|Speed of sound |:x:| :x: |
|Phase-equilibrium condition|:x:| :x: |

### Region 4

| Property | Implemented | Tested |
|:---------|:--------------:|:-------:|
|Saturation pressure |:heavy_check_mark:| :heavy_check_mark: |
|Saturation temperature |:heavy_check_mark:| :heavy_check_mark: |

### Region 5

- **Forward equations**

| Property | Implemented | Tested |
|:---------|:--------------:|:-------:|
|Specific volume|:x:| :x: |
|Specific internal energy|:x:| :x: |
|Specific entropy|:x:| :x: |
|Specific enthalpy|:x:| :x: |
|Specific isobaric heat capacity |:x:| :x: |
|Specific isochoric heat capacity |:x:| :x: |
|Speed of sound |:x:| :x: |
