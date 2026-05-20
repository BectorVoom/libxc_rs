//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1379/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1379<F: Float>(t10110: F, t1528: F, t1902: F, t1912: F, t20936: F, t25188: F, t259: F, t2718: F, t28311: F, t4268: F, t5636: F, t5637: F, t5657: F, t67339: F, t7537: F, t855: F, t87898: F, t87915: F, t99010: F, t99022: F, t99036: F) -> F {
    let t105723 = -F::cast_from(0.12337005501361698274e-1_f64) * t99022 - F::new(3.0) * t99010 * t1528 - F::cast_from(0.78134368175290755733e-1_f64) * t87898 - F::cast_from(0.24674011002723396547e-1_f64) * t87915 - F::new(3.0) * t67339 * t1912 + F::new(6.0) * t855 * t2718 * t7537 * t5657 + F::new(6.0) * t25188 * t5637 + t20936 * t1902 * t259 + F::cast_from(0.49348022005446793095e-1_f64) * t99036 - F::new(18.0) * t855 * t10110 * t7537 * t5636 - F::new(18.0) * t4268 * t28311;
    t105723
}
