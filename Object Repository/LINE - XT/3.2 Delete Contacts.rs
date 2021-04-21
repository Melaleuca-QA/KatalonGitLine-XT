<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>3.2 Delete Contacts</name>
   <tag></tag>
   <elementGuidId>0a5583fb-5156-4df3-9c09-cc49ab9273d4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
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
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>https://xttwapi.melaleuca.com/Profile/1.2/Contacts/${CustomerId}?LineUid=${Contact}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.G_CustomerID</defaultValue>
      <description></description>
      <id>62a56df4-9282-4300-bfbd-a4490839bba7</id>
      <masked>false</masked>
      <name>CustomerId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G2_TokenKey</defaultValue>
      <description></description>
      <id>3658b827-792d-415e-ad23-fd9160200c67</id>
      <masked>false</masked>
      <name>TokenKey</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G3_Contact</defaultValue>
      <description></description>
      <id>ee8427ee-babf-464e-8405-f5b877047646</id>
      <masked>false</masked>
      <name>Contact</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G3_SpouseContact</defaultValue>
      <description></description>
      <id>c14a320d-710b-4ce5-8b7f-f5d5605156ca</id>
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
