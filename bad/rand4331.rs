
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4331(_: S2, _: S3) {}
        
        fn test4331() { foo4331(S1, S2, S4, S5, S6); }
    