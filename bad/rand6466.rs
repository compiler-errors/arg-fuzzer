
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6466(_: S5, _: S7, _: S5) {}
        
        fn test6466() { foo6466(S1, S1, S5, S5, S6, S5); }
    