
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5029(_: S1, _: S7, _: S8) {}
        
        fn test5029() { foo5029(S1, S1, S4, S7, S2); }
    