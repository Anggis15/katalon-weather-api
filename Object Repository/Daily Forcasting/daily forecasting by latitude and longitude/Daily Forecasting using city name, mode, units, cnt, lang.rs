<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Daily Forecasting using city name, mode, units, cnt, lang</name>
   <tag></tag>
   <elementGuidId>d3663a31-a712-44a8-84dd-e791ae8e7e68</elementGuidId>
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
   <restUrl>https://api.openweathermap.org/data/2.5/forecast?lat=${latitude}&amp;lon=${longitude}&amp;mode=xml&amp;unit=imperial&amp;cnt=${cnt}&amp;lang=KR&amp;appid=${appId}</restUrl>
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
      <id>ad07b75c-e24d-447c-850f-4932a6bf15cb</id>
      <masked>false</masked>
      <name>latitude</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.longitude</defaultValue>
      <description></description>
      <id>ff6b2d26-3758-472a-ab60-2dfae50ccb7a</id>
      <masked>false</masked>
      <name>longitude</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cnt</defaultValue>
      <description></description>
      <id>34fc9f5a-c22b-42cd-93d9-c60c7d57baa4</id>
      <masked>false</masked>
      <name>cnt</name>
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
