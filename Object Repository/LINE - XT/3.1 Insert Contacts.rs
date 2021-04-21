<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>3.1 Insert Contacts</name>
   <tag></tag>
   <elementGuidId>ad3e30a8-2aa9-41fd-8169-9d2c47e5be4f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;CustomerId\&quot;: ${CustomerId},\n\&quot;Contact\&quot;:{\&quot;LineUid\&quot;:\&quot;${Contact}\&quot;},\n\&quot;SpouseContact\&quot;:{\&quot;LineUid\&quot;:\&quot;${SpouseContact}\&quot;}\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${TokenKey}</value>
   </httpHeaderProperties>
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://xttwapi.melaleuca.com/Profile/1.2/Contacts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.G_TokenKey</defaultValue>
      <description></description>
      <id>e76cc5ec-3aba-427c-a577-7d3cf66142d7</id>
      <masked>false</masked>
      <name>TokenKey</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_CustomerID</defaultValue>
      <description></description>
      <id>6f584952-8fa1-4f39-a589-8d1d3ece04f9</id>
      <masked>false</masked>
      <name>CustomerId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G3_Contact</defaultValue>
      <description></description>
      <id>a44192ff-c0c0-4360-acb8-689e34ec8d28</id>
      <masked>false</masked>
      <name>Contact</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G3_SpouseContact</defaultValue>
      <description></description>
      <id>ad395899-313b-406b-a8a2-a6a216fed2b7</id>
      <masked>false</masked>
      <name>SpouseContact</name>
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
