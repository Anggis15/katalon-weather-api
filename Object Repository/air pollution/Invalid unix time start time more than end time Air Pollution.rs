<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Invalid unix time start time more than end time Air Pollution</name>
   <tag></tag>
   <elementGuidId>b2800a08-fc40-440b-91ba-fc94ed42d2f7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>10.0.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://api.openweathermap.org/data/2.5/air_pollution/forecast?lat=${lat}&amp;lon=${lon}&amp;start=1735785029&amp;end=1734636629&amp;appid=${appId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.appId</defaultValue>
      <description></description>
      <id>137b2d17-ebc3-431a-be0c-b0a4bc72d3cb</id>
      <masked>false</masked>
      <name>appId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.latitude</defaultValue>
      <description></description>
      <id>5af98d2d-9b48-45ea-ae5c-9c1db9c3c66c</id>
      <masked>false</masked>
      <name>lat</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.longitude</defaultValue>
      <description></description>
      <id>f9348a0c-8efd-4a9a-9674-736f37aebb45</id>
      <masked>false</masked>
      <name>lon</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
