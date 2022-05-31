
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5880(_: S4, _: S3, _: S8, _: S7, _: S6, _: S1, _: S5) {}
        
        fn test5880() { foo5880(S4, S1, S1, S5); }
    