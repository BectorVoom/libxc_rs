//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2301/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2301<F: Float>(t210: F, t24848: F, t27505: F, t24574: F, t27466: F, t3507: F, t8054: F, t27455: F, t1409: F, t24849: F, t24851: F, t24853: F, t24860: F, t27406: F, t27460: F, t27725: F, t3248: F, t3252: F, t3493: F, t3604: F, t3610: F, t3612: F, t7283: F, t7362: F, t7376: F, t85984: F, t85986: F) -> (F, F) {
    let t95092 = t27505 * t210 * t24848;
    let t95098 = F::cast_from(0.18277045187202515961e-2_f64) * t24574 * t27466;
    let t95109 = t8054 * t3507;
    let t95114 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27455;
    let t95122 = F::cast_from(0.14621636149762012769e-1_f64) * t95092 * t24853 + F::cast_from(0.14621636149762012769e-1_f64) * t27406 * t24860 - t95098 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t7362 * t27460 * t3252 - F::cast_from(0.54831135561607547884e-2_f64) * t7283 * t7362 * t27460 * t3248 + F::cast_from(2.0_f64) * t3604 * t27725 + F::cast_from(2.0_f64) * t3610 * t95109 * t3612 - t95114 + F::cast_from(0.54831135561607547884e-2_f64) * t85984 - F::cast_from(0.27415567780803773942e-2_f64) * t24849 * t24851 * t1409 * t3493 * t7376 + F::cast_from(0.36554090374405031922e-2_f64) * t85986;
    (t95109, t95122)
}
