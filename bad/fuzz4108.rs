
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4108(_: S8, _: S5, _: S3, _: S8) {}
        
        fn test4108() { foo4108(S4, S7, S7, S6, S8, S6); }
    