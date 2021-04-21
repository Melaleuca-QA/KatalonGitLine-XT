<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>1.2 Search Customer</name>
   <tag></tag>
   <elementGuidId>490eccd9-1aa1-4b71-9ff3-cdb78ac3e987</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${Tokenkey}</value>
   </httpHeaderProperties>
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://xttwapi.melaleuca.com/Profile/1.2/Customers/Search?Melaleucaid=${Melaleucaid}&amp;PhoneNumber=${PhoneNumber}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.G_MelaleucaId</defaultValue>
      <description></description>
      <id>064fff16-cc8d-4c2b-bead-158df1120b7b</id>
      <masked>false</masked>
      <name>Melaleucaid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_PhoneNumber</defaultValue>
      <description></description>
      <id>36f3a849-8250-4a95-96bd-ae6639c1e659</id>
      <masked>false</masked>
      <name>PhoneNumber</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_TokenKey</defaultValue>
      <description></description>
      <id>913df5f0-eedb-4128-9af1-04f6827f71cf</id>
      <masked>false</masked>
      <name>Tokenkey</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_CustomerID</defaultValue>
      <description></description>
      <id>47272951-907e-4265-be7d-1d58ce64a11b</id>
      <masked>false</masked>
      <name>CustomerID</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
