using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace Labyrinthmm
{
    public class Lang
    {
        public enum Token { Wall, Space, A, B, NewLine }

        public static Token[] Lex(string code)
        {
            string[] lines = code.Split('\n');
            LineType[] lineTypes = GetLineTypes(lines);

            int startLine, endLine;
            DetectCode(lineTypes, out startLine, out endLine);

            List<Token> tokens = new List<Token>();

            for (int i = startLine; i <= endLine; i++)
            {
                foreach (char c in lines[i])
                {
                    switch (c)
                    {
                        case '#': tokens.Add(Token.Wall); break;
                        case ' ': tokens.Add(Token.Space); break;
                        case 'A': tokens.Add(Token.A); break;
                        case 'B': tokens.Add(Token.B); break;
                    }
                }

                tokens.Add(Token.NewLine);
            }

            return tokens.ToArray();
        }

        enum LineType { Empty, Code, Comment }

        static LineType[] GetLineTypes(string[] lines)
        {
            LineType[] lineTypes = new LineType[lines.Length];
            Regex regex = new Regex(@"^[# AB]+$");

            for (int i = 0; i < lines.Length; i++)
            {
                if (lines[i].Length == 0)
                    lineTypes[i] = LineType.Empty;
                else if (regex.IsMatch(lines[i]))
                    lineTypes[i] = LineType.Code;
                else
                    lineTypes[i] = LineType.Comment;
            }

            return lineTypes;
        }

        static void DetectCode(LineType[] lineTypes, out int startLine, out int endLine)
        {
            startLine = Array.IndexOf(lineTypes, LineType.Code);
            endLine = Array.LastIndexOf(lineTypes, LineType.Code);

            if ((startLine > 0 && lineTypes[startLine - 1] == LineType.Comment) ||
                (endLine < lineTypes.Length - 1 && lineTypes[endLine + 1] == LineType.Comment))
            {
                throw new Exception("No empty line between comments and code");
            }

            for (int i = startLine; i <= endLine; i++)
            {
                if (lineTypes[i] == LineType.Empty)
                    throw new Exception("Empty line between the code");
                else if (lineTypes[i] == LineType.Comment)
                    throw new Exception("Comment line between the code");
            }
        }
    }
}
