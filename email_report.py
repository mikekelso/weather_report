import yagmail
yag = yagmail.SMTP(user = 'mmichaelkkelso@gmail.com', host = 'smtp.gmail.com',port=587, smtp_starttls=True, smtp_ssl=False)


#

yag.send(to='mmichaelkkelso@gmail.com', subject ='weather report' , contents = '/home/mike/rust_weather/output.txt')


yag.close()