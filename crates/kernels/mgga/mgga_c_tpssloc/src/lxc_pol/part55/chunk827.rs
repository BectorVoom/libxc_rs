//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 827/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk827<F: Float>(t5: F, t2020: F, t8690: F, t1873: F, t7423: F, t2108: F, t131: F, t8308: F, t8302: F, t112: F) -> (F, F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t8691 = t8690 * t2020;
    let t8699 = t7423 * t1873;
    let t8854 = t2108 * t2108;
    let t8855 = t8854 * t131;
    let t8856 = t8855 * t8308;
    let t8859 = piecewise3::<F>(t8, F::new(0.0), F::new(5.0) / F::new(144.0) * t8302 * t8856);
    let t8860 = t8859 * t112;
    (t8691, t8699, t8854, t8855, t8856, t8859, t8860)
}
