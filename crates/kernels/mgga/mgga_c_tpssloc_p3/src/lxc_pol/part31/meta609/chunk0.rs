//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1854/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1854<F: Float>(t90963: F, t90970: F, t90983: F, t90987: F, t1338: F, t27051: F, t91010: F, t91113: F, t91120: F, t91135: F, t91137: F, t91140: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t93590 = F::cast_from(0.15352717957250113407e0_f64) * t90963;
    let t93592 = F::cast_from(0.76763589786250567036e-1_f64) * t90970;
    let t93599 = F::cast_from(0.16449340668482264365e-1_f64) * t90983;
    let t93600 = F::cast_from(0.16449340668482264365e-1_f64) * t90987;
    let t93607 = t1338 * t27051;
    let t93618 = F::cast_from(0.15352717957250113407e0_f64) * t91010;
    let t93633 = F::new(7.0) / F::new(288.0) * t91113;
    let t93636 = F::new(7.0) / F::new(576.0) * t91120;
    let t93644 = F::new(7.0) / F::new(144.0) * t91135;
    let t93645 = F::new(7.0) / F::new(144.0) * t91137;
    let t93646 = F::cast_from(0.80745512188280781706e-3_f64) * t91140;
    (t93590, t93592, t93599, t93600, t93607, t93618, t93633, t93636, t93644, t93645, t93646)
}
