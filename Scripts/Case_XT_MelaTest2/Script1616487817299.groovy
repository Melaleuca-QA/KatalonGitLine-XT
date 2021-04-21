import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

response1 = WS.sendRequest(findTestObject('LINE - XT/2.1 XT - Get Password Access Token', [('Tokenkey2') : GlobalVariable.G2_TokenKey]))
println('.. response1 is: .. ' + response1)

def slurper1 = new groovy.json.JsonSlurper()
def result1 = slurper1.parseText(response1.getResponseBodyContent())
def value1 = result1.access_token

GlobalVariable.G2_TokenKey = value1
println('.. G2_TokenKey is: .. ' + value1)

response2 = WS.sendRequest(findTestObject('LINE - XT/2.2 .Get Customer Detail', [('TokenKey') : GlobalVariable.G2_TokenKey, ('CustomerId') : GlobalVariable.G2_CustomerId
            , ('FirstName') : GlobalVariable.G2_FirstName, ('LastName') : GlobalVariable.G2_LastName, ('Email') : GlobalVariable.G2_Email
            , ('PhoneNumber') : GlobalVariable.G2_PhoneNumber, ('MelaleucaId') : GlobalVariable.G2_MelaleucaId, ('EnrollmentDate') : GlobalVariable.G2_EnrollmentDate
            , ('ActiveStatus') : GlobalVariable.G2_ActiveStatus, ('VipStatus') : GlobalVariable.G2_VipStatus, ('Commitment') : GlobalVariable.G2_Commitment
            , ('AvailableLsd') : GlobalVariable.G2_AvailableLsd, ('CustomerType') : GlobalVariable.G2_CustomerType]))
println('.. response2 is: .. ' + response2)

def slurper2 = new groovy.json.JsonSlurper()
def result2 = slurper2.parseText(response2.getResponseBodyContent())
def value2_1 = result2.CustomerId
def value2_2 = result2.FirstName
def value2_3 = result2.LastName
def value2_4 = result2.Email
def value2_5 = result2.PhoneNumber
def value2_6 = result2.MelaleucaId
def value2_7 = result2.EnrollmentDate
def value2_8 = result2.ActiveStatus
def value2_9 = result2.VipStatus
def value2_a = result2.Commitment
def value2_b = result2.AvailableLsd
def value2_c = result2.CustomerType

GlobalVariable.G2_CustomerId = value2_1

println('.. CustomerId is: .. ' + value2_1)
println('.. FirstName is: .. ' + value2_2)
println('.. LastName is: .. ' + value2_3)
println('.. Email is: .. ' + value2_4)
println('.. PhoneNumber is: .. ' + value2_5)
println('.. MelaleucaId is: .. ' + value2_6)
println('.. EnrollmentDate is: .. ' + value2_7)
println('.. ActiveStatus is: .. ' + value2_8)
println('.. VipStatus is: .. ' + value2_9)
println('.. Commitment is: .. ' + value2_a)
println('.. AvailableLsd is: .. ' + value2_b)
println('.. CustomerType is: .. ' + value2_c)

response3 = WS.sendRequest(findTestObject('LINE - XT/2.3 Get Promotions'))
println('.. response3 is: .. ' + response3)

