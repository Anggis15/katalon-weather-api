<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Daily Forecasting using cityName, stateCode, countryId and lang</name>
   <tag></tag>
   <elementGuidId>e35f9be0-16e6-4038-973a-d9ae8190c1f5</elementGuidId>
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
   <restUrl>https://api.openweathermap.org/data/2.5/forecast?q=${cityName},${stateCode},${countryId}&amp;lang=KR&amp;appid=${appId}</restUrl>
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
      <defaultValue>GlobalVariable.cityName</defaultValue>
      <description></description>
      <id>833548a3-6dd1-4c88-a14f-47a44a44f538</id>
      <masked>false</masked>
      <name>cityName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.countryCode</defaultValue>
      <description></description>
      <id>e8db8378-e272-471b-9c94-2cf3a4716989</id>
      <masked>false</masked>
      <name>countryId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.stateCode</defaultValue>
      <description></description>
      <id>47fc942d-1eb0-4030-b0d3-11b50a31c118</id>
      <masked>false</masked>
      <name>stateCode</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()




WS.verifyElementPropertyValue(response, 'cod', '200')
WS.verifyElementPropertyValue(response, 'message', '0')

String cityName = GlobalVariable.cityName

WS.verifyElementPropertyValue(response, 'city.name', cityName)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
