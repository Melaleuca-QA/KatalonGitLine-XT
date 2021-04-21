<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>2.2 .Get Customer Detail</name>
   <tag></tag>
   <elementGuidId>e4ed8601-8139-4052-989d-8bfa2e266a32</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${TokenKey}</value>
   </httpHeaderProperties>
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://xttwapi.melaleuca.com/Profile/1.2/Customers/${CustomerId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.G2_TokenKey</defaultValue>
      <description></description>
      <id>5e99d701-91c6-463c-bf7f-926f93f7e16f</id>
      <masked>false</masked>
      <name>TokenKey</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G2_CustomerId</defaultValue>
      <description></description>
      <id>0abb3f33-34b5-421c-9842-5e469c13033c</id>
      <masked>false</masked>
      <name>CustomerId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G2_FirstName</defaultValue>
      <description></description>
      <id>d00af51a-9983-4c2e-bdc4-7e29d8387f5a</id>
      <masked>false</masked>
      <name>FirstName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G2_LastName</defaultValue>
      <description></description>
      <id>3a0bdc53-548f-4887-b49d-746500a12f00</id>
      <masked>false</masked>
      <name>LastName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G2_Email</defaultValue>
      <description></description>
      <id>0c698810-0cf1-4af0-affe-e53e78fc4736</id>
      <masked>false</masked>
      <name>Email</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G2_PhoneNumber</defaultValue>
      <description></description>
      <id>912ba5dc-2f16-425b-9e26-83b564c2be88</id>
      <masked>false</masked>
      <name>PhoneNumber</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G2_MelaleucaId</defaultValue>
      <description></description>
      <id>b8e48d19-d995-4c52-b200-d5d65370e4b6</id>
      <masked>false</masked>
      <name>MelaleucaId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G2_EnrollmentDate</defaultValue>
      <description></description>
      <id>f0a76483-0f3a-4f98-aa8e-1b0d00d63230</id>
      <masked>false</masked>
      <name>EnrollmentDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G2_ActiveStatus</defaultValue>
      <description></description>
      <id>be7ae8cc-d0ab-4cf7-b8ba-b8eaa6210d20</id>
      <masked>false</masked>
      <name>ActiveStatus</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G2_VipStatus</defaultValue>
      <description></description>
      <id>26ce5886-d536-4b90-b0d9-b7c3fa270b81</id>
      <masked>false</masked>
      <name>VipStatus</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G2_Commitment</defaultValue>
      <description></description>
      <id>34209d36-8726-4618-a34d-9ff8396a543b</id>
      <masked>false</masked>
      <name>Commitment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G2_AvailableLsd</defaultValue>
      <description></description>
      <id>c3be4a77-4de4-4dab-baad-46b82c52ee32</id>
      <masked>false</masked>
      <name>AvailableLsd</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G2_CustomerType</defaultValue>
      <description></description>
      <id>02c2678c-b464-4b23-bc63-2debe164e3df</id>
      <masked>false</masked>
      <name>CustomerType</name>
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
