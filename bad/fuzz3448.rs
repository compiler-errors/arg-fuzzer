
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3448(_: S3, _: S4) {}
        
        fn test3448() { foo3448(S2, S3, S5, S8); }
    