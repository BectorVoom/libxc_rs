//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 823/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk823<F: Float>(t74741: F, t74743: F, t74745: F, t74749: F, t74751: F, t74756: F, t74759: F, t74762: F, t74765: F, t74768: F, t71097: F, t71109: F, t71112: F, t68910: F, t74753: F, t74772: F, t74775: F) -> (F,) {
    let t77183 = 0.18183107769496894487e-1 * t74741;
    let t77184 = 0.23268647941669485538e-4 * t74743;
    let t77185 = 0.2553875993597870364e-4 * t74745;
    let t77186 = 0.2553875993597870364e-4 * t74749;
    let t77187 = 0.1702583995731913576e-4 * t74751;
    let t77189 = 0.40911992481368012595e-1 * t74756;
    let t77190 = 0.5454932330849068346e-1 * t74759;
    let t77191 = 0.8182398496273602519e-1 * t74762;
    let t77192 = 0.13637330827122670865e0 * t74765;
    let t77193 = 0.5454932330849068346e-1 * t74768;
    let t77195 = 0.99317399751028291929e-5 * t71097;
    let t77196 = 0.12414674968878536491e-4 * t71109;
    let t77197 = 0.29795219925308487579e-4 * t71112;
    let t77200 = t77183 - t77184 - t77185 + t77186 + t77187 + 0.17451485956252114154e-4 * t74753 + t77189 - t77190 + t77191 - t77192 - t77193 - 0.13139479569676025391e-5 * t74772 - t77195 - t77196 + t77197 - 0.4379826523225341797e-6 * t74775 - 0.16566831523319392755e-1 * t68910;
    (t77200,)
}
