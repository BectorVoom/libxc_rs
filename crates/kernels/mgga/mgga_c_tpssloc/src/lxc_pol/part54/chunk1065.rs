//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1065/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1065<F: Float>(t25: F, t265: F, t394: F, t26806: F, t1409: F, t2064: F, t26775: F, t3966: F, t40: F, t607: F, t7131: F, t7865: F, t1081: F, t1649: F, t1877: F, t2057: F, t24191: F, t24339: F, t2522: F, t25892: F, t25898: F, t25901: F, t25905: F, t25921: F, t25928: F, t25930: F, t25934: F, t25938: F, t25945: F, t26563: F, t26740: F, t26744: F, t26756: F, t26774: F, t28: F, t6841: F, t6848: F, t7110: F, t7114: F, t7649: F, t7656: F, t7845: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t26807 = piecewise3::<f64>(t395, F::new(0.0), t26806);
    let t26814 = piecewise3::<f64>(t115, t26775, t7131 * t1409 / F::new(2.0) + t2064 * t3966 / F::new(2.0) + t26807 * t40 / F::new(2.0) + t7865 * t607 / F::new(2.0));
    let t26861 = F::new(3.0) * t26563 * t25892 + F::new(3.0) / F::new(2.0) * t2522 * t7110 * t7649 - F::new(3.0) / F::new(2.0) * t24191 * t25898 + F::new(3.0) / F::new(2.0) * t2522 * t2057 * t25901 + F::new(3.0) / F::new(2.0) * t2522 * t2057 * t25905 + F::new(3.0) / F::new(2.0) * t2522 * t7845 * t6841 + t1877 * t26740 * t28 / F::new(2.0) - t1877 * t26744 * t6848 / F::new(2.0) + t1877 * t7845 * t1081 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t24191 * t25921 - t1877 * t24339 * t7656 / F::new(2.0) + t26756 * t25928 - t1877 * t7114 * t25930 / F::new(2.0) - t1877 * t7114 * t25934 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t2522 * t2057 * t25938 + t1877 * t7110 * t1649 / F::new(2.0) - t1877 * t7114 * t25945 / F::new(2.0) - t26774;
    (t26814, t26861)
}
