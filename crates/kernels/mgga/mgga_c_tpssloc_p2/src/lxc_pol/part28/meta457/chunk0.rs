//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1662/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1662<F: Float>(t25: F, t265: F, t394: F, t24379: F, t2064: F, t2250: F, t24355: F, t40: F, t607: F, t7131: F, t1081: F, t1877: F, t2057: F, t23781: F, t23789: F, t23792: F, t23796: F, t23807: F, t23810: F, t23813: F, t24191: F, t24335: F, t24339: F, t24344: F, t2522: F, t28: F, t3231: F, t4314: F, t6841: F, t6848: F, t7110: F, t7114: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t24380 = piecewise3::<F>(t395, F::cast_from(0.0_f64), t24379);
    let t24387 = piecewise3::<F>(t115, t24355, t24380 * t40 / F::cast_from(2.0_f64) + t7131 * t607 + t2064 * t2250 / F::cast_from(2.0_f64));
    let t24419 = F::cast_from(3.0_f64) * t4314 * t2057 * t23781 + F::cast_from(3.0_f64) * t2522 * t7110 * t6841 - F::cast_from(3.0_f64) * t24191 * t23789 + F::cast_from(3.0_f64) * t2522 * t2057 * t23792 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t2057 * t23796 + t1877 * t24335 * t28 / F::cast_from(2.0_f64) - t1877 * t24339 * t6848 + t1877 * t7110 * t1081 + t1877 * t24344 * t23807 - t1877 * t7114 * t23810 - t1877 * t7114 * t23813 / F::cast_from(2.0_f64) + t1877 * t2057 * t3231 / F::cast_from(2.0_f64);
    (t24380, t24387, t24419)
}
