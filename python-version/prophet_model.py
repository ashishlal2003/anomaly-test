from prophet import Prophet

def fit_prophet_model(df):
    model = Prophet()
    model.fit(df)
    return model

def forecast_prophet_model(model, periods):
    future = model.make_future_dataframe(periods=periods)
    forecast = model.predict(future)
    return forecast

