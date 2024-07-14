import pandas as pd
from prophet import Prophet

# Load data
file_path = "C:\\Users\\ashis\\Downloads\\frontend-logs.csv"
data = pd.read_csv(file_path)

# Preprocess data
data['ds'] = pd.to_datetime(data['datetime'], format='%Y-%m-%dT%H:%M:%S.%fZ')  # Replace 'date_column' with the correct column name
data['y'] = data['status'].astype(float)   # Replace 'value_column' with the correct column name

# Fit model
model = Prophet()
model.fit(data)

# Make future dataframe
future = model.make_future_dataframe(periods=10)
forecast = model.predict(future)

print(forecast[['ds', 'yhat', 'yhat_lower', 'yhat_upper']])
