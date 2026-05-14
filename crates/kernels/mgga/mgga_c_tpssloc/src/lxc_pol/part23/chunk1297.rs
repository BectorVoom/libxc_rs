//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1297/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1297<F: Float>(t22229: F, t4869: F, t6084: F, t1164: F, t3400: F, t3403: F, t21939: F, t4874: F, t1156: F, t3375: F, t63332: F, t63334: F, t63361: F, t71142: F, t71144: F, t71146: F, t71152: F, t77989: F, t77992: F, t77995: F, t78057: F) -> (F, F, F, F, F, F) {
    let t78242 = 0.4155806185363551302e3 * t4869 * t22229;
    let t78243 = t6084 * t6084;
    let t78247 = 0.51947577317044391277e2 * t1164 * t3400 * t78243 * t3403;
    let t78250 = 0.46785788981077169656e1 * t1164 * t4874 * t21939;
    let t78254 = 0.35089341735807877242e1 * t1164 * t3375 * t78243 * t1156;
    let t78266 = -0.31659259259259259258e-1 * t63332 + 0.47488888888888888888e-1 * t63334 + 0.47488888888888888888e-1 * t71142 - 0.14246666666666666667e0 * t71144 + 0.94977777777777777776e-1 * t63361 - 0.42739999999999999999e0 * t78057 - 0.26382716049382716049e-1 * t71146 + 0.4274e0 * t77989 + 0.17808333333333333333e-1 * t77992 - 0.52765432098765432099e-1 * t77995 - 0.14246666666666666667e0 * t71152;
    (t78242, t78243, t78247, t78250, t78254, t78266)
}
