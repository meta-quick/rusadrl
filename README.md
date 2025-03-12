# rusadrl
This is rust implementation of ODRL 2.2 by following the specification at
https://www.w3.org/TR/odrl-model/

### Validation reference
- https://www.w3.org/2016/poe/wiki/Validation

### Policy Inference

![alt text](image.png)

#### IncludedIn
```xml
odrl:play odrl:includedIn odrl:present . 
odrl:display odrl:includedIn odrl:present . 
odrl:print odrl:includedIn odrl:present .
```
```xml
<http://example.com/policy:01a>
    a odrl:Policy;
    odrl:permission [
        a odrl:Permission ;
        odrl:target ex:PartA ;
        odrl:action odrl:present ;
        odrl:assignee ex:Bob
    ] ;
    odrl:prohibition [
        a odrl:Prohibition ;
        odrl:target ex:PartB ;
        odrl:action odrl:print ;
        odrl:assignee ex:Bob
    ] .
```
Inferred
```xml
<http://example.com/policy:01b>
    a odrl:Policy;
    odrl:permission [
        a odrl:Permission ;
        odrl:target ex:PartA ;
        odrl:action odrl:present ;
        odrl:action odrl:play ; 
        odrl:action odrl:display; 
        odrl:action odrl:print ; 
        odrl:assignee ex:Bob
    ] ;
    odrl:prohibition [
        a odrl:Prohibition ;
        odrl:target ex:PartB ;
        odrl:action odrl:print ;
        odrl:assignee ex:Bob
    ] .
```

#### Semantics of Party Relations

```xml
ex:Bob odrl:partOf ex:W3C . 
ex:Alice odrl:partOf ex:W3C .
```
Raw
```xml
<http://example.com/policy:02a>
    a odrl:Policy;
    odrl:permission [
        a odrl:Permission ;
        odrl:target ex:PartA ;
        odrl:action odrl:play;
        odrl:assignee ex:W3C
    ] .
```
Inferred
```xml
<http://example.com/policy:02b>
    a odrl:Policy;
    odrl:permission [
        a odrl:Permission ;
        odrl:target ex:PartA ;
        odrl:action odrl:play;
        odrl:assignee ex:W3C ;
	    odrl:assignee ex:Alice ;
	    odrl:assignee ex:Bob 
    ] .
```

### Semantics of Asset Relations

```xml
ex:PartA odrl:partOf ex:Dataset1 . 
ex:PartB odrl:partOf ex:Dataset1 .
```

Raw
```xml
<http://example.com/policy:03a>
    a odrl:Policy;
    odrl:permission [
        a odrl:Permission ;
        odrl:target ex:Dataset1 ;
        odrl:action odrl:play ;
        odrl:assignee ex:Alice
    ] .
```
Inferred
```xml
<http://example.com/policy:03b>
    a odrl:Policy;
    odrl:permission [
        a odrl:Permission ;
        odrl:target ex:Dataset1 ;
        odrl:target ex:PartA ;
        odrl:target ex:PartB ;
        odrl:action odrl:play ;
        odrl:assignee ex:Alice
    ] .
```