//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 818/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk818<F: Float>(t74562: F, t74574: F, t74577: F, t74579: F, t74581: F, t74584: F, t74590: F, t15616: F, t2106: F, t2145: F, t14683: F, t8577: F, t74557: F, t74569: F, t74587: F, t74594: F, t74598: F, t74600: F, t74603: F, t74605: F) -> (F,) {
    let t77105 = 0.29085809927086856922e-4 * t74562;
    let t77107 = 0.23268647941669485538e-4 * t74574;
    let t77108 = 0.1276937996798935182e-4 * t74577;
    let t77109 = 0.85129199786595678799e-5 * t74579;
    let t77110 = 0.85129199786595678799e-5 * t74581;
    let t77111 = 0.85129199786595678799e-5 * t74584;
    let t77113 = 0.2627895913935205078e-5 * t74590;
    let t77116 = t2145 * t15616 * t2106;
    let t77117 = 0.90915538847484472429e-2 * t77116;
    let t77118 = t8577 * t14683;
    let t77119 = 0.42564599893297839398e-5 * t77118;
    let t77120 = 0.17451485956252114154e-4 * t74557 + t77105 + 0.10511583655740820313e-5 * t74569 + t77107 - t77108 - t77109 - t77110 - t77111 + 0.17519306092901367188e-6 * t74587 - t77113 - 0.15372131649401827111e-4 * t74594 + t77117 + t77119 - t74598 - t74600 - t74603 - t74605;
    (t77120,)
}
