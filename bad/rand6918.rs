
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6918(_: S1, _: S2, _: S3, _: S5, _: S6) {}
        
        fn test6918() { foo6918(S0, S3, S3, S0, S3, S2, S5); }
    