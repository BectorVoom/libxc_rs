//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3106/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3106<F: Float>(t50846: F, t50848: F, t50853: F, t63918: F, t63921: F, t63924: F, t63927: F, t63930: F, t63933: F, t63936: F, t63939: F, t63997: F, t64003: F, t64006: F, t64009: F) -> F {
    let t64358 = -F::cast_from(0.8585111111111111111e-1_f64) * t63918 - F::new(0.5519e-1) * t63921 - F::new(0.27595e-1) * t63924 - F::new(0.16557e0) * t63927 + F::cast_from(0.36793333333333333333e-1_f64) * t63930 + F::new(0.44152e0) * t63933 + F::new(0.49671e0) * t63936 + F::new(0.198684e1) * t63939 + F::new(0.258925e1) * t63997 - F::cast_from(0.49057777777777777779e0_f64) * t50846 - F::new(0.11038e0) * t50848 + F::cast_from(0.36793333333333333334e0_f64) * t50853 - F::new(0.66228e0) * t64003 + F::new(0.198684e1) * t64006 + F::new(0.16504875e0) * t64009;
    t64358
}
