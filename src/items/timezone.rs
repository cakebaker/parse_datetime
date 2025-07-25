// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use winnow::ModalResult;

use super::time;

pub(crate) fn parse(input: &mut &str) -> ModalResult<time::Offset> {
    time::timezone(input)
}
