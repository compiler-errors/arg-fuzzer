
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9595(_: S2, _: S4) {}
        
        fn test9595() { foo9595(S6, S2, S1, S5, S3); }
    