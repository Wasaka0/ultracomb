from scipy import signal
import matplotlib.pyplot as plt
import numpy as np
import math

# This script is used to desing an elliptic filter to be used in the frequency shifter implemented on Ultracomb using the "Third" or "Weaver" Method.
# The filter used in this approach has a  cut-off frequency of fs/4.
# Since the cut-off frequency depends on the sampling frequency (fs) this means that the coefficients for the filter stay the same for any given fs.
# The script prints the IIR coefficients for a biquad cascade implementation compatible with code at src/audio/designed_filter.rs => EllipFs4::initialize().
# Additionally it can plot its frequency response and group delay.

# Filter parameters
order = 16
rp = 0.5 # Max pass-band ripple
rs = 60 # Min attenuation in stop-band
fs = 48000
fc = fs/4
# Display options
plots_active = True
# Limits the frequency range of the plot to the area around fc
focus_on_fc = False 
fc_disp_range = 100

# Calculate filter Biquad cascade coefficients
sos = signal.ellip(order, rp, rs, fc, 'low', fs=fs, output='sos')
print('Fs='+str(fs))
# Print coeffs in format compatible to code at src/audio/
for i in range(0,int(math.ceil(order/2))):
    print('self.cascade.coeffs(' + str(i),end='')
    for coeff in range(0,6):
        if coeff != 3: # Skip a0 since all coeffs are normalized to it (a0 = 1.0 always)
            print(', ' + str(sos[i][coeff]),end='')
    print(');')

# Plot frequency response
if plots_active:
    b, a = signal.ellip(order, rp, rs, fc, 'low', fs=fs, output='ba')
    w, h = signal.freqz(b, a, fs=fs)
    plt.semilogx(w, 20 * np.log10(abs(h)))
    plt.title('Elliptic filter frequency response (rp='+str(rp)+' rs='+str(rs)+' fc='+str(fc)+' fs='+str(fs)+')')
    plt.xlabel('Frequency [Hz]')
    plt.ylabel('Amplitude [dB]')
    plt.xlim(2,fs/2)
    if focus_on_fc:
        plt.title('Elliptic filter frequency response detail at fc (rp='+str(rp)+' rs='+str(rs)+' fc='+str(fc)+' fs='+str(fs)+')')
        plt.axvline(fc, color='green') # cutoff frequency
        plt.xlim(fc - fc_disp_range,fc + fc_disp_range)
    plt.ylim(-rs - 20, 10)
    plt.grid(which='both', axis='both')
    plt.show()

# According to https://csoundjournal.com/ezine/summer2000/processing/index.html phase response does not affect the performance of the frequency shifter
# Plot group delay
if plots_active:
    w, gd = signal.group_delay((b, a),fs=fs)
    plt.plot(w, gd)
    plt.title('Elliptic filter group dealy (rp='+str(rp)+' rs='+str(rs)+' fc='+str(fc)+' fs='+str(fs)+')')
    plt.axvline(fc, color='green') # cutoff frequency
    plt.xlim(2,fs/2)
    if focus_on_fc:
        plt.title('Elliptic filter frequency response detail at fc (rp='+str(rp)+' rs='+str(rs)+' fc='+str(fc)+' fs='+str(fs)+')')
        plt.axvline(fc, color='green') # cutoff frequency
        plt.xlim(fc - fc_disp_range,fc + fc_disp_range)
    plt.xlabel('Frequency [Hz]')
    plt.ylabel('Phase [rad]')
    plt.grid(which='both', axis='both')
    plt.show()