import numpy as np

def calculated_residuals(df, forecast):
    df_merged = df.merge(forecast[['ds', 'yhat']],on='ds')
    df_merged['residual'] = df_merged['y'] - df_merged['yhat']
    return df_merged

def calculate_z_scores(residuals):
    mean_residual = np.mean(residuals)
    std_residual = np.std(residuals)
    z_scores = (residuals - mean_residual) / std_residual
    return z_scores

def detect_anomalies(z_scores, threshold=3):
    return np.where(np.abs(z_scores) > threshold)[0]