
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5546(_: S1, _: S2, _: S3, _: S4, _: S5, _: S6) {}
        
        fn test5546() { foo5546(S3, S8, S5, S1, S5, S2, S1); }
    