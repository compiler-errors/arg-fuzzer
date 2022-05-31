
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1043(_: S8, _: S4, _: S1, _: S2, _: S8) {}
        
        fn test1043() { foo1043(S1, S6, S3, S8, S7, S2, S5); }
    