//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1146/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1146<F: Float>(t33: F, t1497: F, t9936: F, t2: F, t3225: F, t1201: F, t12715: F, t22: F, t2829: F, t3226: F, t4388: F, t4391: F, t555: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t12795 = t9936 * t1497;
    let t12798 = t3225 * t2;
    let t12808 = piecewise3::<F>(t34, F::new(0.0), F::new(8.0) / F::new(27.0) * t12795 * t3226 + F::new(8.0) / F::new(9.0) * t12798 * t12715 - F::new(2.0) / F::new(9.0) * t4388 * t2829 - F::new(4.0) / F::new(3.0) * t1201 * t555 + F::new(4.0) * t4391 * t22);
    t12808
}
