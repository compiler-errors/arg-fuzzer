
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8755(_: S1, _: S4, _: S5, _: S8) {}
        
        fn test8755() { foo8755(S3, S5, S2, S1, S2, S6); }
    