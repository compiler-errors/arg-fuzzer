
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13694(_: S1, _: S2, _: S4, _: S7) {}
        
        fn test13694() { foo13694(S4, S4, S1, S2, S6); }
    