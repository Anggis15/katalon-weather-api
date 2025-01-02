<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Daily Forecasting using cityName, countryCode, mode, units, cnt, and lang</name>
   <tag></tag>
   <elementGuidId>82f9f58c-f47a-40b7-8102-18f474a32fa4</elementGuidId>
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
   <restUrl>https://api.openweathermap.org/data/2.5/forecast?q=${cityName},${countryId}&amp;mode=xml&amp;unit=imperial&amp;cnt=${cnt}&amp;lang=KR&amp;appid=${appId}</restUrl>
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
      <defaultValue>GlobalVariable.cnt</defaultValue>
      <description></description>
      <id>34fc9f5a-c22b-42cd-93d9-c60c7d57baa4</id>
      <masked>false</masked>
      <name>cnt</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.countryCode</defaultValue>
      <description></description>
      <id>e8db8378-e272-471b-9c94-2cf3a4716989</id>
      <masked>false</masked>
      <name>countryId</name>
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