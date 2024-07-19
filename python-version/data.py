import pandas as pd

import pandas as pd

def load_and_preprocess_data(filepath, status_code=404, freq='2s'):
    df = pd.read_csv(filepath, parse_dates=['datetime'])

    df_filtered = df[df['status'] == status_code]
    df_filtered['datetime'] = df_filtered['datetime'].dt.tz_localize(None)
    df_filtered = df_filtered.set_index('datetime').resample(freq).size().reset_index(name='y')
    df_filtered.rename(columns={'datetime': 'ds'}, inplace=True)
    print(df_filtered)
    return df_filtered