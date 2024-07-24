from data import load_and_preprocess_data
from prophet_model import fit_prophet_model, forecast_prophet_model
from anomaly_detection import calculated_residuals, calculate_z_scores, detect_anomalies

def main():

    ## PUT IN YOUR FILEPATH OF FRONTEND LOGS HERE!
    ##CAN LATER CHANGE IT TO RELATIVE PATH WITHIN CODEBASE FOR DEVELOPMENT
    filepath = "frontend-logs.csv"
    df = load_and_preprocess_data(filepath, status_code=404)

    model = fit_prophet_model(df)

    forecast = forecast_prophet_model(model, periods=30)
    df_with_residuals = calculated_residuals(df, forecast)

    z_scores = calculate_z_scores(df_with_residuals['residual'])

    anomalies = detect_anomalies(z_scores)

    print(f"Anomalies detected at: {df_with_residuals['ds'].iloc[anomalies].values}")

if __name__ == "__main__":
    main()