<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Search Customer</name>
   <tag></tag>
   <elementGuidId>70ece602-4704-4a5e-879a-36ef60d9d0b9</elementGuidId>
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
      <value>Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6IjFFODRGOEREOTE0NzE2NDBBOEE5Mjk2NkJCMzlDNkYxRjMzQUZDMjQiLCJ0eXAiOiJKV1QiLCJ4NXQiOiJIb1Q0M1pGSEZrQ29xU2xtdXpuRzhmTTZfQ1EifQ.eyJuYmYiOjE2MTcyNzE2MTMsImV4cCI6MTYxNzI3NTIxMywiaXNzIjoiaHR0cHM6Ly94dGlkZW50aXR5dHcubWVsYWxldWNhLmNvbSIsImF1ZCI6WyJodHRwczovL3h0aWRlbnRpdHl0dy5tZWxhbGV1Y2EuY29tL3Jlc291cmNlcyIsIkN1c3RvbWVyUHJvZmlsZUFQSSJdLCJjbGllbnRfaWQiOiJsaW5lLWNvb2xiZS1jbGllbnRjcmVkZW50aWFscy1kZXYiLCJkYXRhX3BvbGljeSI6Ilt7XCJBUElcIjpcIlByb2ZpbGUuQVBJXCIsIFwiQ29udHJvbGxlclwiOlwiQ3VzdG9tZXJzXCIsIFwiQWN0aW9uXCI6XCJTZWFyY2hcIiwgXCJLZXlzXCI6W1wiQ3VzdG9tZXJJRFwiXX1dIiwianRpIjoiNjhjNTQ4NDY3ZGNkOGFjNzBhMDUzNWJmYmY1NzhmMTAiLCJzY29wZSI6WyJQcm9maWxlLkFQSS5Db250YWN0cy5Qb3N0Q29udGFjdHMuTW9kaWZ5IiwiUHJvZmlsZS5BUEkuQ3VzdG9tZXJzLlNlYXJjaC5SZWFkIl19.HrvI4qQZKPmsYGYmUQehmRaOyBZ6xbvAySQTXOfeS-IbPwL7qD5-KPJi4xk8QTGqh5iIoQ2Tr-j7s0mijPCwx3P5efzdKbwAG-9DV1G9nr3BIiLjAAXku3H0JZ9YY2lKa0f7c89nvveso4sBPFJXvMzrlOKIeUk-KvDGhpic4vC7wNrZkA7wDIm3x9DW89awsWwsWiGFXs74-8fcY9WOsQmCxxPA71HBv5rSKBCdVlbYJqI5GRUhGn1Vjzt990iKvN8P0tEMXgwzcSgXCc2u0lw8IBDZSQ6zU9Dp0XRv45G2mjzVQZ_gYrjiNwW6mB5nM1r0ngzAVHIptb_K5egdfA</value>
   </httpHeaderProperties>
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://xttwapi.melaleuca.com/Profile/1.2/Customers/Search?MelaleucaId=12105136142&amp;PhoneNumber=0937984541</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
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
